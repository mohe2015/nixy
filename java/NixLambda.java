interface NixLambda extends NixValue {

	// TODO FIXME maybe split up into two subclasses lambda and lazy?
	// this *could* maybe help us with type-safety

	// should not be allowed to be NixLazy
	NixValue call(NixLazy arg);

	static NixLazy createFunction(NixLambda function) {
		return () -> {
			return (NixLambda) (arg) -> {
				NixLambda.ensureLambda(arg);
				return function.call(arg);
			};
		};
	}

	static void ensureLambda(NixLazy arg) {
		if (arg == null) {
			throw new IllegalArgumentException("This is a lambda. Therefore you need to pass a parameter.");
		}
	}
}