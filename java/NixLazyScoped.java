import java.util.ArrayDeque;
import java.util.IdentityHashMap;

public abstract class NixLazyScoped implements NixLazy {

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

	ArrayDeque<NixAttrset> scopes;
	ArrayDeque<NixAttrset> withs;

	public NixLazyScoped(ArrayDeque<NixAttrset> scopes, ArrayDeque<NixAttrset> withs) {
		this.scopes = scopes;
		this.withs = withs;
	}

}
