unsafe extern "C" fn reflet_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn reflet_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_module_accessor = fighter.global_table[0x5].get_ptr() as *mut FighterModuleAccessor;
    fighter.clear_lua_stack();
    let app_fighter: *mut Fighter = std::mem::transmute(sv_system::battle_object(fighter.lua_state_agent));
    let current_point = WorkModule::get_float(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CURRENT_POINT);
    smash::app::FighterSpecializer_Reflet::change_hud_kind(app_fighter, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_LW, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_REFLET_SPECIAL_AIR_LW, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    WorkModule::set_int(fighter.module_accessor, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    WorkModule::set_int(fighter.module_accessor, *GROUND_CORRECT_KIND_AIR, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_OBJECT_ID);
    AttackModule::set_overlap_hit(fighter.module_accessor, true);
    if current_point > 0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
        smash::app::FighterSpecializer_Reflet::change_grimoire(fighter_module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
        smash::app::FighterSpecializer_Reflet::set_flag_to_table(fighter_module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_lw_main_loop as *const () as _))
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
    }
}

unsafe extern "C" fn reflet_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_module_accessor = fighter.global_table[0x5].get_ptr() as *mut FighterModuleAccessor;
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    let catch_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_WORK_INT_CATCH_STATUS);
    let mut status = 0;
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_GRAB_IS_GRAB, 0);
    sv_module_access::grab(fighter.lua_state_agent);
    if is_changing {
        if !smash::app::FighterSpecializer_Reflet::check_special_lw_pos(fighter_module_accessor) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 0);
            sv_module_access::grab(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
            sv_module_access::grab(fighter.lua_state_agent);
        }
    }
    if !is_changing {
        if MotionModule::is_end(fighter.module_accessor) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
                if catch_status == *FIGHTER_REFLET_STATUS_SPECIAL_LW_CATCH_STATUS_CATCH_START {
                    status = *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE;
                }
            }
            else {
                status = *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END;
            }
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
    }
    fun_7100007080(fighter, is_changing.into());
    0.into()
}

unsafe extern "C" fn reflet_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[0xB].get_i32();
    if status_kind != *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE {
        if CatchModule::is_catch(fighter.module_accessor) {
            CatchModule::catch_cut(fighter.module_accessor, false, false);
        }
    }
    0.into()
}