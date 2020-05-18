fn main() -> rays::RayResult<()> {
    let config = rays::load_json::<rays::Config>("config.json")?;
    rays::run_file(&config, "scene.json")
}
