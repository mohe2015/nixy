import java.util.*;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

public class MainClosure extends NixLazyScoped {

	public MainClosure(ArrayDeque<NixAttrset> scopes, ArrayDeque<NixAttrset> withs) {
		super(scopes, withs);
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure(new ArrayDeque<>(List.of((NixAttrset) globals.force())), new ArrayDeque<>()).force().call(NixInteger.create(5)));
	}

	public NixValue force() {

		// let binding

		NixAttrset let = (NixAttrset) NixAttrset.create(new HashMap<>()).force();

		// this idea seems to work
		return new NixLazyScoped(addToScope(scopes, let), withs) {

			@Override
			public NixValue force() {
				((NixString)NixString.create("""
hi""").add().createCall().force()).value.intern()
				let.value.put("a", new NixLazy() {
					@Override
					public NixValue force() {
						return findVariable(scopes, withs, "b").force();
					}
				});
				let.value.put("b", new NixLazy() {

					@Override
					public NixValue force() {
						return NixInteger.create(5).force();
					}
				});

				return (arg) -> arg.add(findVariable(scopes, withs, "a")).force();
			}
		}.force();
	}

	public ArrayDeque<NixAttrset> addToScope(final ArrayDeque<NixAttrset> scopes, NixAttrset value) {
		ArrayDeque<NixAttrset> newScopes = scopes.clone();
		newScopes.add(value);
		return newScopes;
	}
}
