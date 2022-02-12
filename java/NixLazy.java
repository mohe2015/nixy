public interface NixLazy {

	NixValue force();

	static NixLazy createIf(NixLazy condition, NixLazy trueCase, NixLazy falseCase) {
		return () -> ((NixBoolean) condition.force()).value ? trueCase.force() : falseCase.force();
	}

	default NixLazy add(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixInteger.create(((NixInteger) this.force()).value + ((NixInteger) second.force()).value).force();
	}

	default NixLazy subtract(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixInteger.create(((NixInteger) this.force()).value - ((NixInteger) second.force()).value).force();
	}

	default NixLazy multiply(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixInteger.create(((NixInteger) this.force()).value * ((NixInteger) second.force()).value).force();
	}

	default NixLazy divide(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixInteger.create(((NixInteger) this.force()).value / ((NixInteger) second.force()).value).force();
	}

	default NixLazy lt(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(((NixInteger) this.force()).value < ((NixInteger) second.force()).value).force();
	}

	default NixLazy lte(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(((NixInteger) this.force()).value <= ((NixInteger) second.force()).value).force();
	}

	default NixLazy gt(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(((NixInteger) this.force()).value > ((NixInteger) second.force()).value).force();
	}

	default NixLazy gte(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(((NixInteger) this.force()).value >= ((NixInteger) second.force()).value).force();
	}

	default NixLazy eq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(this.force().equals(second.force())).force();
	}

	default NixLazy neq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(!this.force().equals(second.force())).force();
	}
}
