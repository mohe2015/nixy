import java.util.*;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class MainClosure implements NixLazy {

	public static void main(String[] args) {
		System.out.println(new MainClosure().force().call(NixInteger.create(5)));
	}

	public NixValue force() {
		//scopes.push((NixAttrset) globals.force());

		// let binding

		NixAttrset let = (NixAttrset) NixAttrset.create(new HashMap<>()).force();

		let.value.put("a", new NixLazy() {
			@Override
			public NixValue force() {
				return findVariable(finalScopes, withs, "b").force();
			}
		});
		let.value.put("b", () -> NixInteger.create(5).force());

		ArrayDeque<NixAttrset> finalScopes1 = scopes;
		NixValue returnValue = (arg) -> arg.add(findVariable(finalScopes1, withs, "a")).force();


		return returnValue;
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
