public class MainClosure extends NixLazyScoped {

	public MainClosure(java.util.ArrayDeque<NixAttrset> scopes, java.util.ArrayDeque<NixAttrset> withs) {
		super(scopes, withs);
	}

	public NixValue force() {
		return (new NixLazy() {

			@Override
			public NixValue force() {
				/* head */

				NixAttrset let = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

				return new NixLazyScoped(addToScope(scopes, let), withs) {

					@Override
					public NixValue force() {


						let.value.put(((NixString) NixString.create("""
								hi""").add().createCall().force()).value.intern(), NixInteger.create(1).createCall());

						/* body */
						return findVariable(scopes, withs, "hi").createCall().force();
					}
				}.force();
			}
		}).force();
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure(new java.util.ArrayDeque<>(java.util.List.of((NixAttrset) globals.force())), new java.util.ArrayDeque<>()).force());
	}
}