macro_rules! define_stat_tables {
	(
		$VarsEnum: ident {
			$( $VarName: ident ),*
		    $(,)?
	    }

		TRAIT: $Trait: ident;
		TABLE: $Table: ident;
		VALUE_TABLE: $ValueTable: ident;
	) => {
		#[derive(Serialize, Deserialize)]
		#[derive(PartialEq, Eq, Hash)]
		#[derive(Debug, Clone, Copy)]
		pub enum $VarsEnum {
			$( $VarName ,)*
		}

		impl $VarsEnum {
			pub const ALL: &[$VarsEnum] = &[
				$( $VarsEnum::$VarName ,)*
			];
		}

		pub trait $Trait {
			fn as_enum() -> $VarsEnum;
		}

		$(
			impl $Trait for $VarName {
				fn as_enum() -> $VarsEnum {
					$VarsEnum::$VarName
				}
			}
		)*

		mod type_table {
			use super::*;

			declarative_type_state::type_table! {
				#[derive(Serialize, Deserialize, Clone)]
				pub struct $Table {
				    $( $VarName  : $VarName   ,)*
			    }
			}
		}

		mod value_table {
			use super::*;

			declarative_type_state::type_value_table! {
				#[derive(Serialize, Deserialize)]
				pub struct $ValueTable<S> {
				    $( $VarName   ,)*
			    }
			}
		}
	};
}

pub(crate) use define_stat_tables;

macro_rules! define_num_stats {
	($($Name:ident[$Min: expr, $Max: expr]),* $(,)?) => {
		$(
			comfy_bounded_ints::new_bound_signed!($Name(i64)[$Min, $Max]);
		)*
	};
}

pub(crate) use define_num_stats;
