use bottlerocket_settings_plugin::SettingsPlugin;
use model::{BootSettings, KubernetesSettings};
use model_derive::model;

#[derive(SettingsPlugin)]
#[model(rename = "settings", impl_default = true)]
struct AwsK8sSettings {
    motd: settings_extension_motd::MotdV1,
    kubernetes: KubernetesSettings,
    updates: settings_extension_updates::UpdatesSettingsV1,
    host_containers: settings_extension_host_containers::HostContainersSettingsV1,
    bootstrap_containers: settings_extension_bootstrap_containers::BootstrapContainersSettingsV1,
    ntp: settings_extension_ntp::NtpSettingsV1,
    network: settings_extension_network::NetworkSettingsV1,
    kernel: settings_extension_kernel::KernelSettingsV1,
    boot: BootSettings,
    aws: settings_extension_aws::AwsSettingsV1,
    metrics: settings_extension_metrics::MetricsSettingsV1,
    pki: settings_extension_pki::PkiSettingsV1,
    container_registry: settings_extension_container_registry::RegistrySettingsV1,
    oci_defaults: settings_extension_oci_defaults::OciDefaultsV1,
    oci_hooks: settings_extension_oci_hooks::OciHooksSettingsV1,
    cloudformation: settings_extension_cloudformation::CloudFormationSettingsV1,
    dns: settings_extension_dns::DnsSettingsV1,
    container_runtime: settings_extension_container_runtime::ContainerRuntimeSettingsV1,
    autoscaling: settings_extension_autoscaling::AutoScalingSettingsV1,
    nvidia_timeslicing: settings_extension_nvidia_timeslicing::NvidiaTimeslicingV1,
}
