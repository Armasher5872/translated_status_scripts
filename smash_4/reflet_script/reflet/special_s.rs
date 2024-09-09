unsafe extern "C" fn reflet_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn reflet_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[0x5].get_ptr() as *mut FighterModuleAccessor;
    let situation_kind = fighter.global_table[0x16].get_i32();
    fighter.clear_lua_stack();
    let app_fighter: *mut Fighter = std::mem::transmute(sv_system::battle_object(fighter.lua_state_agent));
    let current_point = WorkModule::get_float(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
    FighterSpecializer_Reflet::change_hud_kind(app_fighter, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    WorkModule::set_int(fighter.module_accessor, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    WorkModule::set_int(fighter.module_accessor, *GROUND_CORRECT_KIND_AIR, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    if current_point > 0.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
        WorkModule::set_float(fighter.module_accessor, current_point-1.0, FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
        if current_point <= 0.0 {
            FighterSpecializer_Reflet::set_flag_to_table(module_accessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
        FighterSpecializer_Reflet::change_grimoire(module_accessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE);
    }
    else {
        FighterSpecializer_Reflet::change_grimoire(module_accessor, -1);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        let kinetic = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
        let correct = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        let kinetic = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
        let correct = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if !is_changing {
        if !fun_7100022f90(fighter).get_bool() {
            return 1.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
            if 12.0 <= frame {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_FLAG_MAGIC_EMPTY_EFFECT_DONE) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_FOLLOW, hash40("reflet_book_smoke"), hash40("handl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
                    sv_module_access::effect(fighter.lua_state_agent);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_FLAG_MAGIC_EMPTY_EFFECT_DONE);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_GIGAFIRE, false, -1);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_TRY);
        }
    }
    fun_7100007080(fighter, is_changing.into());
    0.into()
}

unsafe extern "C" fn fun_7100022f90(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn reflet_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}