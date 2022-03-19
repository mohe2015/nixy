import java.util.ArrayDeque;
import java.util.Deque;
import java.util.IdentityHashMap;
import java.util.Map;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

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
	protected static NixLazy globals = NixAttrset.create(new IdentityHashMap<>() {{
		this.put("builtins", builtins_);
		this.put("import", import_);
		this.put("true", true_);
		this.put("false", false_);
	}});

	public NixLazy findVariable(Deque<NixAttrset> scopes, Deque<NixAttrset> withs, String name) {
		Iterable<NixAttrset> scopesIterable = scopes::descendingIterator;
		Stream<NixAttrset> scopesStream = StreamSupport.stream(scopesIterable.spliterator(), false);

		Iterable<NixAttrset> withsIterable = withs::descendingIterator;
		Stream<NixAttrset> withsStream = StreamSupport.stream(withsIterable.spliterator(), false);

		return Stream
				.concat(scopesStream, withsStream)
				.flatMap(nixAttrset -> nixAttrset.value.entrySet().stream())
				.filter(entry -> {
					System.out.println(entry);
					return entry.getKey().equals(name);
				})
				.findFirst()
				.map(Map.Entry::getValue)
				.orElseThrow();
	}

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
