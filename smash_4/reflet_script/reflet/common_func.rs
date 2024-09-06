unsafe extern "C" fn fun_7100007080(fighter: &mut L2CFighterCommon, is_changing: L2CValue) -> L2CValue {
    let status_kind_interrupt = fighter.global_table[0x9].get_i32();
    let situation_kind = fighter.global_table[0x16].get_i32();
    let prev_situation_kind = fighter.global_table[0x17].get_i32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let correct_ground = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    let kinetic_ground = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    let correct_air = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    let kinetic_air = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    let motion_kind_ground = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    let motion_kind_air = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    let special_n_air_invoke_fall_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_air_invoke_fall_speed_mul"));
    let special_n_air_invoke_speed_y_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_air_invoke_speed_y_limit"));
    let mut rate = 1.0;
    if !is_changing.get_bool() {
        rate = MotionModule::rate(fighter.module_accessor);
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct_ground));
            KineticModule::change_kinetic(fighter.module_accessor, kinetic_ground);
            if motion_kind != motion_kind_ground {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind_ground), 0.0, rate, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_kind_air), -1.0, 1.0, 0.0, false, false);
                MotionModule::set_rate(fighter.module_accessor, rate);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct_air));
            KineticModule::change_kinetic(fighter.module_accessor, kinetic_air);
            if motion_kind != motion_kind_air {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind_air), 0.0, rate, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion_kind_air), -1.0, 1.0, 0.0, false, false);
                MotionModule::set_rate(fighter.module_accessor, rate);
            }
            if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD].contains(&status_kind_interrupt) {
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_n_air_invoke_fall_speed_mul*-0.089);
                if special_n_air_invoke_speed_y_limit <= get_sum_speed_y {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_n_air_invoke_speed_y_limit);
                }
            }
        }
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT, 
            *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END,
            *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END
        ].contains(&status_kind_interrupt) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT, hash40("sys_landing_smoke"), hash40("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, false);
            sv_module_access::effect(fighter.lua_state_agent);
        }
    }
}