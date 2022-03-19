import java.util.*;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class MainClosure extends NixLazy {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		ArrayDeque<NixAttrset> scopes = new ArrayDeque<>();
		ArrayDeque<NixAttrset> withs = new ArrayDeque<>();
		scopes.push((NixAttrset) globals.force()); // do this only in the root

		// let binding

		NixAttrset let = (NixAttrset) NixAttrset.create(new HashMap<>()).force();

		let.value.put("a", new NixLazy(addToScope(scopes, let), withs) {
			@Override
			public NixValue force() {
				return findVariable(scopes, withs, "b").force();
			}
		});
		let.value.put("b", new NixLazy(addToScope(scopes, let), withs) {

					@Override
					public NixValue force() {
						return NixInteger.create(5).force();
					}
				});

		NixValue returnValue = (arg) -> arg.add(findVariable(addToScope(scopes, let), withs, "a")).force();

		return returnValue;
	}

	public ArrayDeque<NixAttrset> addToScope(final ArrayDeque<NixAttrset> scopes, NixAttrset value) {
		ArrayDeque<NixAttrset> newScopes = scopes.clone();
		newScopes.add(value);
		return newScopes;
	}

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
}
