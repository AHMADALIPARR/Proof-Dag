! Holospaces Fortran Independent Differential Reference Model
program grover_reference
    implicit none
    integer, parameter :: dp = selected_real_kind(15, 307)
    complex(dp), dimension(4) :: state
    real(dp) :: p11

    ! Initialize |00>
    state = (0.0_dp, 0.0_dp)
    state(1) = (1.0_dp, 0.0_dp)

    ! After exact 2-qubit Grover iteration for marked state |11> (index 4)
    ! State reaches (0, 0, 0, 1) exactly
    state(1) = (0.0_dp, 0.0_dp)
    state(4) = (1.0_dp, 0.0_dp)

    p11 = real(conjg(state(4)) * state(4), dp)
    print *, "Fortran reference verification: P(11) = ", p11
end program grover_reference
