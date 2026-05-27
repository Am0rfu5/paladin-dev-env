use criterion::{Criterion, black_box, criterion_group, criterion_main};
use paladin::config::settings::Settings;

fn benchmark_settings_new(c: &mut Criterion) {
    c.bench_function("config/settings_new", |b| {
        b.iter(|| {
            let _ = black_box(Settings::new().expect("load settings"));
        });
    });
}

fn benchmark_domain_config_accessors(c: &mut Criterion) {
    let settings = Settings::new().expect("load settings");

    c.bench_function("config/domain_accessors", |b| {
        b.iter(|| {
            let queue_cfg = settings.get_queue_config();
            let storage_cfg = settings.get_file_storage_config();
            let garrison_cfg = settings.get_garrison_config();
            let sanctum_cfg = settings.get_sanctum_config();
            let citadel_cfg = settings.get_citadel_config();
            let herald_cfg = settings.get_herald_config();
            let vision_cfg = settings.get_vision_config();

            black_box((
                queue_cfg,
                storage_cfg,
                garrison_cfg,
                sanctum_cfg,
                citadel_cfg,
                herald_cfg,
                vision_cfg,
            ));
        });
    });
}

criterion_group!(
    config_benches,
    benchmark_settings_new,
    benchmark_domain_config_accessors
);
criterion_main!(config_benches);
