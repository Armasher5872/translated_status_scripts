unsafe extern "C" fn bayonetta_attack_air_f_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX, true);
    fighter.status_attach_wall_wait();
    0.into()
}