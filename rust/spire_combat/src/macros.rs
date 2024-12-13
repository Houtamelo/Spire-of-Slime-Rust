macro_rules! positions {
	("✔️🛑🛑🛑") => {
		crate::internal_prelude::PositionMatrix([true, false, false, false])
	};
	("🛑✔️🛑🛑") => {
		crate::internal_prelude::PositionMatrix([false, true, false, false])
	};
	("🛑🛑✔️🛑") => {
		crate::internal_prelude::PositionMatrix([false, false, true, false])
	};
	("🛑🛑🛑✔️") => {
		crate::internal_prelude::PositionMatrix([false, false, false, true])
	};
	("✔️✔️🛑🛑") => {
		crate::internal_prelude::PositionMatrix([true, true, false, false])
	};
	("🛑✔️✔️🛑") => {
		crate::internal_prelude::PositionMatrix([false, true, true, false])
	};
	("🛑🛑✔️✔️") => {
		crate::internal_prelude::PositionMatrix([false, false, true, true])
	};
	("✔️🛑🛑✔️") => {
		crate::internal_prelude::PositionMatrix([true, false, false, true])
	};
	("🛑✔️🛑✔️") => {
		crate::internal_prelude::PositionMatrix([false, true, false, true])
	};
	("✔️🛑✔️🛑") => {
		crate::internal_prelude::PositionMatrix([true, false, true, false])
	};
	("🛑✔️✔️✔️") => {
		crate::internal_prelude::PositionMatrix([false, true, true, true])
	};
	("✔️🛑✔️✔️") => {
		crate::internal_prelude::PositionMatrix([true, false, true, true])
	};
	("✔️✔️🛑✔️") => {
		crate::internal_prelude::PositionMatrix([true, true, false, true])
	};
	("✔️✔️✔️🛑") => {
		crate::internal_prelude::PositionMatrix([true, true, true, false])
	};
	("✔️✔️✔️✔️") => {
		crate::internal_prelude::PositionMatrix([true, true, true, true])
	};
	("🛑🛑🛑🛑") => {
		crate::internal_prelude::PositionMatrix([false, false, false, false])
	};
}

pub(crate) use positions;
