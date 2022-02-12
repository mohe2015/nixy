public abstract class NixLazyBase implements NixLazy {

	protected static NixLazy true_ = NixBoolean.create(true);
	protected static NixLazy false_ = NixBoolean.create(false);
	protected static NixLazy import_ = NixLambda.createFunction((arg) -> {
		System.out.printf("would load %s%n", ((NixToString) arg.force()).toNixString().value);

		return NixInteger.create(1).force();
	});
}
