unsafe extern "C" fn donkey_final_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
    0.into()
}

unsafe extern "C" fn donkey_final_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
    AreaModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    WorkModule::set_int64(fighter.module_accessor, hash40("final_start") as i64, *FIGHTER_DONKEY_STATUS_FINAL_WORK_INT_MOTION_KIND_GROUND);
    WorkModule::set_int64(fighter.module_accessor, hash40("final_air_start") as i64, *FIGHTER_DONKEY_STATUS_FINAL_WORK_INT_MOTION_KIND_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_MOT_CHANGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_TO_ATTACK);
    fun_710000d0d0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_final_main_loop as *const () as _))
}

unsafe extern "C" fn fun_710000d0d0(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[0x16].get_i32();
    let motion_kind_ground = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_WORK_INT_MOTION_KIND_GROUND);
    let motion_kind_air = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_WORK_INT_MOTION_KIND_AIR);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_kind_air), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind_air), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_MOT_CHANGE);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_MOT_CHANGE) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_kind_ground), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind_ground), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_MOT_CHANGE);
        }
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
    }
}

unsafe extern "C" fn donkey_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[0x16].get_i32();
    let prev_situation_kind = fighter.global_table[0x17].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_ATTACK_HIT) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_TO_ATTACK) {
            fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK.into(), false.into());
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR)
        || (situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND) {
            fun_710000d0d0(fighter);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_FINAL_FLAG_TO_ATTACK) {
            fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK.into(), false.into());
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL_JUMP_END.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn donkey_final_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[0xB];
    if status_kind != *FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
        fun_710000a330(fighter);
    }
    0.into()
}

unsafe extern "C" fn fun_710000a330(fighter: &mut L2CFighterCommon) {
    if LinkModule::is_linked(fighter.module_accessor, *FIGHTER_LINK_NO_FINAL) {
        LinkModule::send_event_nodes(fighter.module_accessor, *FIGHTER_LINK_NO_FINAL, Hash40::new("final_cancel"), 0);
    }
}