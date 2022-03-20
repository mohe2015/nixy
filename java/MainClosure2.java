public class MainClosure extends NixLazyScoped {

	public MainClosure(java.util.ArrayDeque<NixAttrset> scopes, java.util.ArrayDeque<NixAttrset> withs) {
		super(scopes, withs);
	}

	public NixValue force() {
		return (new NixLazy() {

			@Override
			public NixValue force() {
				/* head */

				NixAttrset rec = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

				return new NixLazyScoped(addToScope(scopes, rec), withs) {

					@Override
					public NixValue force() {



						rec.value.computeIfAbsent("a", k -> .value.computeIfAbsent("b", k -> findVariable(scopes, withs, "a")findVariable(scopes, withs, "c") SELECT .createCall());rec.value.computeIfAbsent("a", k -> (new NixLazy() {

							@Override
							public NixValue force() {
								/* head */

								NixAttrset rec = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

								return new NixLazyScoped(addToScope(scopes, rec), withs) {

									@Override
									public NixValue force() {



										rec.value.computeIfAbsent("c", k -> NixInteger.create(1).createCall()); return rec; }}.force(); }}).createCall()); return rec; }}.force(); }}).createCall().force();
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure(new java.util.ArrayDeque<>(java.util.List.of((NixAttrset) globals.force())), new java.util.ArrayDeque<>()).force());
	}
}