use ustx::{CURRENT_VERSION, Project};

fn sample_yaml() -> &'static str {
    r#"name: Demo
comment: sample
output_dir: Vocal
cache_dir: UCache
ustx_version: "0.3"
resolution: 480
bpm: 120.0
beat_per_bar: 4
beat_unit: 4
expressions: {}
exp_selectors: []
exp_primary: 0
exp_secondary: 1
key: 0
time_signatures: []
tempos: []
tracks: []
voice_parts: []
wave_parts: []
"#
}

#[test]
fn parses_and_maintains_yaml() {
    let project: Project = Project::from_yaml_str(sample_yaml()).expect("parse yaml");
    let serialized = project.to_yaml_string().expect("serialize yaml");
    let project_roundtrip: Project =
        Project::from_yaml_str(&serialized).expect("roundtrip parse yaml");
    assert_eq!(project_roundtrip.name, "Demo");
    assert_eq!(project_roundtrip.comment, "sample");
}

#[test]
fn compat_upgrade_fills_defaults() {
    let project = Project::from_yaml_str_with_compat(sample_yaml()).expect("upgrade");
    assert_eq!(project.exp_selectors.len(), 10);
    assert_eq!(project.tempos.len(), 1);
    assert_eq!(project.time_signatures.len(), 1);
    assert_eq!(project.tempos[0].bpm, 120.0);
    assert_eq!(project.ustx_version, Some(CURRENT_VERSION));
}

#[test]
fn raw_parse_preserves_version_number() {
    let project = Project::from_yaml_str(sample_yaml()).expect("parse");
    let version = project.ustx_version.expect("version present");
    assert_eq!(version.major, 0);
    assert_eq!(version.minor, 3);
    assert_eq!(version.patch, 0);
}
