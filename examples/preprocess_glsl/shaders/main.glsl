//! define HIGH_PRECISION ""
//! include "main" "constants.glsl"

void main() {
    //! ifndef PI
    //! error "constant pi not found"
    //! endif

    //! ifdef HIGH_PRECISION
    //! define MESSANGE "high precision pi constant"
    //! endif

    //! ifndef HIGH_PRECISION
    //! define MESSANGE "low precision pi constant"
    //! endif

    PI // MESSANGE
}