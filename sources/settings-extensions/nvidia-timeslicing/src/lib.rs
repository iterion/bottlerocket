/// nvidia-timeslicing settings allow users to configure the nvidia-k8s-device-plugin.
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use model_derive::model;
use std::convert::Infallible;

#[model(impl_default = true)]
struct NvidiaTimeslicingV1 {
    enabled: bool,
    replicas: u32,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for NvidiaTimeslicingV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that can be parsed as NvidiaTimeslicingV1.
        Ok(())
    }

    fn generate(
        existing_partial: Option<Self::PartialKind>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialKind, Self>> {
        Ok(GenerateResult::Complete(
            existing_partial.unwrap_or_default(),
        ))
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<()> {
        // NvidiaTimeslicingV1 is validated during deserialization.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_nvidia_timeslicing() {
        let generated = NvidiaTimeslicingV1::generate(None, None).unwrap();

        assert_eq!(
            generated,
            GenerateResult::Complete(NvidiaTimeslicingV1 {
                enabled: None,
                replicas: None,
            })
        )
    }

    #[test]
    fn test_serde_nvidia_timeslicing() {
        let input_json = r#"{
            "enabled": true,
            "replicas": 1
        }"#;

        let timeslicing: NvidiaTimeslicingV1 = serde_json::from_str(input_json).unwrap();

        assert_eq!(
            timeslicing,
            NvidiaTimeslicingV1 {
                enabled: Some(true),
                replicas: Some(1),
            }
        );
    }
}
