macro_rules! impl_harness_adapter {
    () => {
        impl crate::harness::HarnessAdapter for crate::adapters::nvidia_api::NvidiaApiAdapter {
            fn id(&self) -> &'static str {
                "nvidia-api"
            }

            fn invocation(
                &self,
                request: &crate::harness::RunRequest,
            ) -> Result<crate::harness::Invocation, crate::harness::HarnessError> {
                crate::adapters::nvidia_api::invocation::build(self, request)
            }

            fn response(
                &self,
                stdout: String,
            ) -> Result<crate::harness::HarnessResponse, crate::harness::HarnessError> {
                crate::adapters::nvidia_api::response::parse(self, stdout)
            }
        }
    };
}
