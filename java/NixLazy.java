import java.util.IdentityHashMap;

public abstract class NixLazy {

	protected static NixLazy true_ = NixBoolean.create(true);
	protected static NixLazy false_ = NixBoolean.create(false);
	protected static NixLazy import_ = NixLambda.createFunction((arg) -> {
		System.out.printf("would load %s%n", ((NixToString) arg.force()).toNixString().value);

		return NixInteger.create(1).force();
	});
	protected static NixLazy builtins_ = NixAttrset.create(new java.util.IdentityHashMap<String, NixLazy>() {{
		this.put("length", NixLambda.createFunction(array -> NixInteger.create(((NixArray) array.force()).array.size()).force()));
	}});
	protected static NixLazy globals = NixAttrset.create(new IdentityHashMap<>() {{
		this.put("builtins", builtins_);
		this.put("import", import_);
		this.put("true", true_);
		this.put("false", false_);
	}});

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

	public NixLazy add() {
		return this;
	}

	public NixLazy add(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				NixValue thisForced = this.force();
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

	public NixLazy subtract(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value - ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value - ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy multiply(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value * ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value * ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy divide(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixFloat.create(((NixNumber) thisForced).toNixFloat().value / ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixInteger.create(((NixInteger) thisForced).value / ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy lt(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value < ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value < ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy lte(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value <= ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value <= ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy gt(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value > ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value > ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy gte(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {

				NixValue thisForced = this.force();
				NixValue secondForced = second.force();
				if (thisForced instanceof NixFloat || secondForced instanceof NixFloat) {
					return NixBoolean.create(((NixNumber) thisForced).toNixFloat().value >= ((NixNumber) secondForced).toNixFloat().value).force();
				}
				return NixBoolean.create(((NixInteger) thisForced).value >= ((NixInteger) secondForced).value).force();
			}
		};
	}

	public NixLazy land(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(((NixBoolean) this.force()).value && ((NixBoolean) second.force()).value).force();
			}
		};
	}

	public NixLazy lor(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(((NixBoolean) this.force()).value || ((NixBoolean) second.force()).value).force();
			}
		};
	}

	public NixLazy eq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(this.force().equals(second.force())).force();
			}
		};
	}

	public NixLazy neq(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return NixBoolean.create(!this.force().equals(second.force())).force();
			}
		};
	}

	public NixLazy createCall(NixLazy second) {
		NixLambda.ensureLambda(second);
		return new NixLazy() {

			@Override
			public NixValue force() {
				return this.force().call(second);
			}
		};
	}

	public NixLazy createCall() {
		return this;
	}

	public NixLazy get(String name) {
		return new NixLazy() {

			@Override
			public NixValue force() {
				return ((NixAttrset) this.force()).value.get(name).force();
			}
		};
	}
}
