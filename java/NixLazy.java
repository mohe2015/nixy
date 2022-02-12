public interface NixLazy {

	NixValue force();

	static NixLazy createIf(NixLazy condition, NixLazy trueCase, NixLazy falseCase) {
		return () -> ((NixBoolean) condition.force()).value ? trueCase.force() : falseCase.force();
	}

	default NixLazy add(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixFloat.create(((NixNumber) thisForced).toNixFloat().value + ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixInteger.create(((NixInteger) thisForced).value + ((NixInteger) secondForced).value).force();
	}

	default NixLazy subtract(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixFloat.create(((NixNumber) thisForced).toNixFloat().value - ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixInteger.create(((NixInteger) thisForced).value - ((NixInteger) secondForced).value).force();
	}

	default NixLazy multiply(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixFloat.create(((NixNumber) thisForced).toNixFloat().value * ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixInteger.create(((NixInteger) thisForced).value * ((NixInteger) secondForced).value).force();
	}

	default NixLazy divide(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixFloat.create(((NixNumber) thisForced).toNixFloat().value / ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixInteger.create(((NixInteger) thisForced).value / ((NixInteger) secondForced).value).force();
	}

	default NixLazy lt(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value < ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixBoolean.create(((NixInteger) thisForced).value < ((NixInteger) secondForced).value).force();
	}

	default NixLazy lte(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value <= ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixBoolean.create(((NixInteger) thisForced).value <= ((NixInteger) secondForced).value).force();
	}

	default NixLazy gt(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value > ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixBoolean.create(((NixInteger) thisForced).value > ((NixInteger) secondForced).value).force();
	}

	default NixLazy gte(NixLazy second) {
		NixLambda.ensureLambda(second);
		NixValue thisForced = this.force();
		NixValue secondForced = second.force();
		if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
			return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value >= ((NixNumber)secondForced).toNixFloat().value);
		}
		return () -> NixBoolean.create(((NixInteger) thisForced).value >= ((NixInteger) secondForced).value).force();
	}

	default NixLazy land(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(((NixBoolean) this.force()).value && ((NixBoolean) second.force()).value).force();
	}

	default NixLazy lor(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(((NixBoolean) this.force()).value || ((NixBoolean) second.force()).value).force();
	}

	default NixLazy eq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(this.force().equals(second.force())).force();
	}

	default NixLazy neq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> NixBoolean.create(!this.force().equals(second.force())).force();
	}

	default NixLazy createCall(NixLazy second) {
		NixLambda.ensureLambda(second);
		return () -> this.force().call(second);
	}

	default NixLazy createCall() {
		return this;
	}
}
