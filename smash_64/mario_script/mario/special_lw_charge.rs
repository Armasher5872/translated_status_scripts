//Remaining statuses: Special Lw Charge Main

unsafe extern "C" fn mario_special_lw_charge_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_lw_charge_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[0xB].get_i32();
    let special_lw_remove_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_remove_frame"));
    let special_lw_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    let special_lw_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_charge_frame"));
    if status_kind != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, 0);
        WorkModule::set_int(fighter.module_accessor, special_lw_remove_frame, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
    }
    if status_kind != *FIGHTER_STATUS_KIND_WAIT {
        if [
            *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT
        ].contains(&status_kind) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_COMMON, Hash40::new("charge_max"));
            sv_module_access::effect(fighter.lua_state_agent);
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    }
    else {
        if special_lw_charge_frame > special_lw_charge {
            return 0.into();
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_COMMON, Hash40::new("charge_max"));
        sv_module_access::effect(fighter.lua_state_agent);
    }
}