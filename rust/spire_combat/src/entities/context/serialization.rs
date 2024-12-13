use super::*;

pub fn serialize_actors<S: Serializer>(
	actors: &HashMap<Id, Ptr<Actor>>,
	serializer: S,
) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
	let map = HashMap::<Id, Actor>::from_iter(actors.iter().map(|(&id, actor)| {
		let actor: &Actor = &**actor;
		(id, actor.clone())
	}));

	map.serialize(serializer)
}

pub fn deserialize_actors<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> std::result::Result<HashMap<Id, Ptr<Actor>>, D::Error> {
	HashMap::<Id, Actor>::deserialize(deserializer)
		.map(|map| HashMap::from_iter(map.into_iter().map(|(id, actor)| (id, Ptr::new(actor)))))
}

pub fn serialize_states<S: Serializer>(
	states: &IndexedMap<Id, Ptr<ActorState>>,
	serializer: S,
) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
	let map = IndexedMap::<Id, ActorState>::from_iter(states.iter().map(|(id, state)| {
		let state: &ActorState = &**state;
		(*id, state.clone())
	}));

	map.serialize(serializer)
}

pub fn deserialize_states<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> std::result::Result<IndexedMap<Id, Ptr<ActorState>>, D::Error> {
	IndexedMap::<Id, ActorState>::deserialize(deserializer)
		.map(|map| IndexedMap::from_iter(map.into_iter().map(|(id, state)| (id, Ptr::new(state)))))
}
