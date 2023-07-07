//!  Provide some implementations for auxiliary encoding.
//!

///  Generate getters for specified members.
#[macro_export]
macro_rules! getters_generate {
  ($fn_name : ident, $ret_value : ident, $ret_type : ty) => {
    pub fn $fn_name(&self) -> &$ret_type {
      &self.$ret_value
    }
  };
}
