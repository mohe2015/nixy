import java.util.ArrayDeque;
import java.util.IdentityHashMap;

public interface NixLazy {

	// nix repl <TAB>
	/*
abort
baseNameOf
?builtins
derivation
derivationStrict
dirOf
+false
fetchGit
fetchMercurial
fetchTarball
fetchTree
fromTOML
+import
isNull
map
null
placeholder
removeAttrs
scopedImport
throw
toString
+true
	 */

	public abstract NixValue force();

	static NixLazy createIf(NixLazy condition, NixLazy trueCase, NixLazy falseCase) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return ((NixBoolean) condition.force()).value ? trueCase.force() : falseCase.force();
			}
		};
	}

	default NixLazy add() {
		return this;
	}

	default NixLazy add(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixString || secondForced instanceof NixString) {
					return NixString.create(((NixToString) thisForced).toNixString().value + ((NixToString) secondForced).toNixString().value).force();
				}
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value + ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value + ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy subtract(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value - ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value - ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy multiply(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value * ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value * ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy divide(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value / ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value / ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy lt(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value < ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value < ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy lte(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value <= ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value <= ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy gt(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value > ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value > ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy gte(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				NixValue thisForced = NixLazy.this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value >= ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value >= ((NixInteger) secondForced).value).force();
			}
		};
	}

	default NixLazy land(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(((NixBoolean) NixLazy.this.force()).value && ((NixBoolean) second.force()).value).force();
			}
		};
	}

	default NixLazy lor(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(((NixBoolean) NixLazy.this.force()).value || ((NixBoolean) second.force()).value).force();
			}
		};
	}

	default NixLazy eq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(NixLazy.this.force().equals(second.force())).force();
			}
		};
	}

	default NixLazy neq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(!NixLazy.this.force().equals(second.force())).force();
			}
		};
	}

	default NixLazy createCall(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixLazy.this.force().call(second);
			}
		};
	}

	default NixLazy createCall() {
		return this;
	}

	default NixLazy get(String name) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return ((NixAttrset) NixLazy.this.force()).value.get(name).force();
			}
		};
	}
}
