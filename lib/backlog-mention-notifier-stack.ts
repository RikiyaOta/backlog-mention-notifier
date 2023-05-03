import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';

export class BacklogMentionNotifierStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new RustFunction(this, 'backlog-mention-notifier', {
      manifestPath: '../Cargo.toml',
      bundling: {
        environment: {}
      }
    });
  }
}
