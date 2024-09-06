//Remaining Scripts: mario_pump_shoot_exec_status, mario_pump_shoot_exec_stop_status

unsafe extern "C" fn mario_pump_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let parent_lr = LinkModule::get_parent_lr(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    PostureModule::set_lr(weapon.module_accessor, parent_lr);
    0.into()
}

unsafe extern "C" fn mario_pump_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pump"), hash40("angle"));
    WorkModule::set_float(weapon.module_accessor, angle, *WEAPON_MARIO_PUMP_STATUS_WORK_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("control"), &Vector3f{x: -angle, y: 0.0, z: 0.0}, 0, 0);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SHOOT_NUM);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
    if !StopModule::is_stop(weapon.module_accessor) {
        fun_710000eaa0(weapon, false.into());
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(fun_710000eaa0 as *const () as _));
    weapon.fastshift(L2CValue::Ptr(mario_pump_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710000eaa0(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    let water_span_frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
    let water_shoot_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SHOOT_NUM);
    let pump_shoot_num = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pump"), hash40("shoot_num"));
    let pump_span_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pump"), hash40("span_frame"));
    if bool_check.get_bool() {
        if water_shoot_num < pump_shoot_num {
            WorkModule::dec_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
            if water_span_frame <= 0 {
                ArticleModule::generate_article(weapon.module_accessor, *WEAPON_MARIO_PUMP_GENERATE_ARTICLE_WATER, false, -1);
                WorkModule::set_int(weapon.module_accessor, pump_span_frame, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SPAN_FRAME);
                WorkModule::inc_int(weapon.module_accessor, *WEAPON_MARIO_PUMP_INSTANCE_WORK_ID_INT_WATER_SHOOT_NUM);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn mario_pump_shoot_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}