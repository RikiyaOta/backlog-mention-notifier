import { Stack, StackProps, aws_apigateway as apigw } from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";

const getEnv = () => process.env.NODE_ENV ?? "dev";

export class BacklogMentionNotifierStack extends Stack {
	constructor(scope: Construct, id: string, props?: StackProps) {
		super(scope, id, props);

		const backlogMentionNotifierFunction = new RustFunction(
			this,
			`backlog-mention-notifier-${getEnv()}`,
			{
				manifestPath: "./Cargo.toml",
				bundling: {
					environment: {},
				},
			},
		);

		const restApi = new apigw.RestApi(this, "backlog-mention-notifier-api", {
			restApiName: `backlog-mention-notifier-${getEnv()}`,
			deployOptions: {
				stageName: getEnv(),
				// TODO: Restrict source IP addresses.
			},
		});

		const slackResource = restApi.root.addResource("slack");
		slackResource.addMethod(
			"POST",
			new apigw.LambdaIntegration(backlogMentionNotifierFunction, {
				// NOTE: Lambda を非同期で呼び出す.
				proxy: false,
				requestParameters: {
					"integration.request.header.X-Amz-Invocation-Type": "'Event'",
				},
				integrationResponses: [{ statusCode: "200" }],
			}),
			{
				// Lambdaを非同期で呼び出した場合のステータスコードは202になるので合わせる
				methodResponses: [{ statusCode: "202" }],
			},
		);
	}
}
