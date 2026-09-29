#[macro_use]
mod adapter;
mod api_key;
mod default;
mod dotenv;
mod invocation;
mod new;
mod nonempty;
mod response;

#[cfg(test)]
mod tests;

/// Translates requests into non-streaming calls to NVIDIA's chat completions API.
#[derive(Debug)]
pub struct NvidiaApiAdapter {
    executable: String,
    api_key: Option<String>,
}

const ENDPOINT: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

impl_harness_adapter!();
