use super::models::*;

pub fn load_weapon_data() {
    let test_data = r#"
{
    "steel_sword": (
        projectile: Key("sword bolt"),
        shoot_pattern: Around ( angle: 1. ),
        attack_speed: 1.
    )
}
    "#;

    let res: WeaponMap = ron::from_str(test_data).unwrap();

    println!("{:?}", res);
}
