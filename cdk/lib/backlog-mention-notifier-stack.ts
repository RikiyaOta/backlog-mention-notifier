import path from "path";
import { Stack, StackProps, aws_apigateway as apigw } from "aws-cdk-lib";
import * as logs from "aws-cdk-lib/aws-logs";
import {
	PolicyDocument,
	PolicyStatement,
	Effect,
	AnyPrincipal,
} from "aws-cdk-lib/aws-iam";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";

const getEnv = () => process.env.NODE_ENV ?? "development";

const packageName = "backlog-mention-notifier" as const;

const BACKLOG_WEBHOOK_SOURCE_IP_ADDRESSES = [
	"54.64.128.240/32",
	"54.178.233.194/32",
	"13.112.1.142/32",
	"13.112.147.36/32",
	"54.238.175.47/32",
	"54.168.25.33/32",
	"52.192.156.153/32",
	"54.178.230.204/32",
	"52.197.88.78/32",
	"13.112.137.175/32",
	"34.211.15.3/32",
	"35.160.57.23/32",
	"54.68.48.106/32",
	"52.88.47.69/32",
	"52.68.247.253/32",
	"18.182.251.152/32",
] as const;

export class BacklogMentionNotifierStack extends Stack {
	constructor(scope: Construct, id: string, props?: StackProps) {
		super(scope, id, props);

		const backlogMentionNotifierFunction = new RustFunction(
			this,
			// ATTENTION: https://github.com/cargo-lambda/cargo-lambda-cdk/issues/10
			packageName,
			{
				//manifestPath: "./Cargo.toml",
				manifestPath: path.join(__dirname, "..", ".."),
				bundling: {
					commandHooks: {
						beforeBundling(inputDir: string, outputDir: string) {
							return [
								`mkdir -p ${outputDir}/config/`,
								`cp config/config.${getEnv()}.json ${outputDir}/config/`,
							];
						},
						afterBundling(_inputDir: string, _outputDir: string): string[] {
							return [];
						},
					},
					environment: {},
				},
				logRetention: logs.RetentionDays.THREE_DAYS,
			},
		);

		const resourcePolicy = new PolicyDocument({
			statements: [
				new PolicyStatement({
					effect: Effect.ALLOW,
					actions: ["execute-api:Invoke"],
					principals: [new AnyPrincipal()],
					resources: ["execute-api:/*/*/*"],
				}),
				new PolicyStatement({
					effect: Effect.DENY,
					actions: ["execute-api:Invoke"],
					principals: [new AnyPrincipal()],
					resources: ["execute-api:/*/*/*"],
					conditions: {
						NotIpAddress: {
							"aws:SourceIp": BACKLOG_WEBHOOK_SOURCE_IP_ADDRESSES,
						},
					},
				}),
			],
		});

		const restApi = new apigw.RestApi(this, "backlog-mention-notifier-api", {
			restApiName: `backlog-mention-notifier-${getEnv()}`,
			deployOptions: {
				stageName: getEnv(),
			},
			policy: resourcePolicy,
		});

		restApi.addGatewayResponse("backlog-mention-notifier-gateway-response", {
			type: apigw.ResponseType.ACCESS_DENIED,
			templates: {
				"application/json":
					'{"statusCode": "403", "type": "$context.error.responseType"}',
			},
		});

		const slackResource = restApi.root.addResource("slack");
		slackResource.addMethod(
			"POST",
			// ちょっとうまくいかないので、同期呼び出して確認してみる。
			new apigw.LambdaIntegration(backlogMentionNotifierFunction),

			//new apigw.LambdaIntegration(backlogMentionNotifierFunction, {
			//	// NOTE: Lambda を非同期で呼び出す.
			//	proxy: false,
			//	requestParameters: {
			//		"integration.request.header.X-Amz-Invocation-Type": "'Event'",
			//	},
			//	integrationResponses: [{ statusCode: "200" }],
			//}),
			//{
			//	// Lambdaを非同期で呼び出した場合のステータスコードは202になるので合わせる
			//	methodResponses: [{ statusCode: "202" }],
			//},
		);
	}
}
