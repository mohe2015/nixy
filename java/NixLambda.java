interface NixLambda<I extends NixValue, O extends NixValue> extends NixValue {

	// TODO FIXME maybe split up into two subclasses lambda and lazy?
	// this *could* maybe help us with type-safety

	static <I extends NixValue, O extends NixValue> NixLazy<NixLambda<I, O>> createFunction(NixLambda<I, O> function) {
		return () -> {
			return (NixLambda<I, O>) (arg) -> {
				NixLambda.ensureLambda(arg);
				return function.call(arg);
			};
		};
	}

	static void ensureLambda(NixLazy<? extends NixValue> arg) {
		if (arg == null) {
			throw new IllegalArgumentException("This is a lambda. Therefore you need to pass a parameter.");
		}
	}

	// should not be allowed to be NixLazy
	O call(NixLazy<I> arg);
}