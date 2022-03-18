public abstract class NixLazyBase implements NixLazy {

	protected static NixLazy true_ = NixBoolean.create(true);
	protected static NixLazy false_ = NixBoolean.create(false);
	protected static NixLazy import_ = NixLambda.createFunction((arg) -> {
		System.out.printf("would load %s%n", ((NixToString) arg.force()).toNixString().value);

		return NixInteger.create(1).force();
	});
	protected static NixLazy builtins_ = NixAttrset.create(new java.util.IdentityHashMap<String, NixLazy>() {{
		this.put("length", NixLambda.createFunction(array -> NixInteger.create(((NixArray) array.force()).array.size()).force()));
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
}
