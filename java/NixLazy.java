public interface NixLazy {

	NixValue force();

	default NixLazy add(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> {
			return NixInteger.create(((NixInteger) this.force()).value + ((NixInteger) second.force()).value).force();
		};
	}

	default NixLazy eq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> {
			return NixBoolean.create(this.force().equals(second.force())).force();
		};
	}

	static NixLazy createIf(NixLazy condition, NixLazy trueCase, NixLazy falseCase) {
		return () -> {
			return ((NixBoolean)condition.force()).value ? trueCase.force() : falseCase.force();
		};
	}
}
