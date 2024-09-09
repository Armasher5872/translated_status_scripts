//Remaining statuses: Special Lw Shoot Main, Special Lw Shoot Init

unsafe extern "C" fn mario_special_lw_shoot_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_lw_remove_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_remove_frame"));
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(fighter.module_accessor, special_lw_remove_frame, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_REMOVE);
    ArticleModule::change_status(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, *WEAPON_MARIO_PUMP_STATUS_KIND_WAIT, 0);
    0.into()
}