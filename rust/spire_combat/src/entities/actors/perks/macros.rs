macro_rules! define_girl_perks {
    (
	    $enum_ident: ident as $main_enum_var: path {
		    $(
		        $( @$ignore: ident )?
		        $var_ident: ident
		        $( ( $($var_tuple: tt)* ) )?
		        $( { $($var_fields: tt)* } )?
		    ),*
		    $(,)?
	    }
    ) => {
	    declarative_type_state::extract_variants! {
		    #[vars(derive(Debug, Clone, Serialize, Deserialize))]
		    pub enum $enum_ident {
			    $(
			        $var_ident
		            $( ( $( $var_tuple  )* ) )?
		            $( { $( $var_fields )* } )?
			    ),*
		    }
	    }

	    declarative_type_state::delegated_enum! {
		    ENUM_OUT: {
			    #[derive(Debug, Clone, Serialize, Deserialize)]
			    #[derive(DiscriminantHash)]
			    pub enum $enum_ident {
				    $( $var_ident ( $var_ident ) ),*
			    }
		    }

			DELEGATES: {
				impl trait IGirlPerk {
					[fn tick(
						&mut self,
						actor: &mut Ptr<Actor>,
						girl: &mut Ptr<Girl>,
						ctx: &mut ActorContext,
						delta_ms: Int,
					) -> PerkTickResult]
				}
			}
	    }

	    $(
	        define_girl_perks! {
		        $( @$ignore )?
		        $var_ident
	        }

	        impl FromEnum<GirlPerk> for $var_ident {
				fn from_enum(value: GirlPerk) -> Option<Self> {
					if let $main_enum_var($enum_ident::$var_ident(var)) = value {
						Some(var)
					} else {
						None
					}
				}
			}

	        impl<'__a> FromEnum<&'__a GirlPerk> for &'__a $var_ident {
	            fn from_enum(value: &'__a GirlPerk) -> Option<Self> {
		            if let $main_enum_var($enum_ident::$var_ident(var)) = value {
			            Some(var)
		            } else {
			            None
		            }
	            }
        	}

            impl<'__a> FromEnum<&'__a mut GirlPerk> for &'__a mut $var_ident {
		        fn from_enum(value: &'__a mut GirlPerk) -> Option<Self> {
			        if let $main_enum_var($enum_ident::$var_ident(var)) = value {
				        Some(var)
			        } else {
				        None
			        }
		        }
            }
	    )*
    };

	//------------------------------------------------------------------------------------------------------------------
	// Generate basic IPerk impl
	(@NO_IMPL $var_ident: ident) => {

	};

	($var_ident: ident) => {
		impl IGirlPerk for $var_ident { }
	};
}

pub(crate) use define_girl_perks;

macro_rules! define_perks {
    (
	    $enum_ident: ident as $main_enum_var: path  {
		    $(
		        $( @$ignore: ident )?
		        $var_ident: ident
		        $( ( $($var_tuple: tt)* ) )?
		        $( { $($var_fields: tt)* } )?
		    ),*
		    $(,)?
	    }
    ) => {
	    declarative_type_state::extract_variants! {
		    #[vars(derive(Debug, Clone, Serialize, Deserialize))]
		    pub enum $enum_ident {
			    $(
			        $var_ident
		            $( ( $( $var_tuple  )* ) )?
		            $( { $( $var_fields )* } )?
			    ),*
		    }
	    }

	    declarative_type_state::delegated_enum! {
		    ENUM_OUT: {
			    #[derive(Debug, Clone, Serialize, Deserialize)]
			    #[derive(DiscriminantHash)]
			    pub enum $enum_ident {
				    $( $var_ident ( $var_ident ) ),*
			    }
		    }

			DELEGATES: {
				impl trait IPerk {
					[fn tick(
						&mut self,
						actor: &mut Ptr<Actor>,
						ctx: &mut ActorContext,
						delta_ms: Int,
					) -> PerkTickResult]
				}
			}
	    }

	    $(
	        define_perks! {
		        $( @$ignore )?
		        $var_ident
	        }

	        impl FromEnum<Perk> for $var_ident {
		        fn from_enum(value: Perk) -> Option<Self> {
			        if let $main_enum_var($enum_ident::$var_ident(var)) = value {
				        Some(var)
			        } else {
				        None
			        }
		        }
	        }

	        impl<'__a> FromEnum<&'__a Perk> for &'__a $var_ident {
	            fn from_enum(value: &'__a Perk) -> Option<Self> {
		            if let $main_enum_var($enum_ident::$var_ident(var)) = value {
			            Some(var)
		            } else {
			            None
		            }
	            }
        	}

	        impl<'__a> FromEnum<&'__a mut Perk> for &'__a mut $var_ident {
		        fn from_enum(value: &'__a mut Perk) -> Option<Self> {
			        if let $main_enum_var($enum_ident::$var_ident(var)) = value {
				        Some(var)
			        } else {
				        None
			        }
		        }
            }
	    )*
    };

	//------------------------------------------------------------------------------------------------------------------
	// Generate basic IPerk impl
	(@NO_IMPL $var_ident: ident) => {

	};

	($var_ident: ident) => {
		impl IPerk for $var_ident { }
	};
}

pub(crate) use define_perks;
