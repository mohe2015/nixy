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
						rec.mutableMerge("a", () -> NixAttrset.create(new java.util.IdentityHashMap<>()).force()).castAttrset().computeIfAbsent("b", k -> () -> findVariable(scopes, withs, "a").castAttrset().get("c").createCall().force());
						rec.mutableMerge("a", () -> (new NixLazy() {

							@Override
							public NixValue force() {
								/* head */

								NixAttrset rec = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

								return new NixLazyScoped(addToScope(scopes, rec), withs) {

									@Override
									public NixValue force() {


										rec.value.computeIfAbsent("c", k -> () -> NixInteger.create(1).createCall().force());
										return rec;
									}
								}.force();
							}
						}).createCall().force());
						return rec;
					}
				}.force();
			}
		}).createCall().force();
	}

	public static void main(String[] args) {
		System.out.println((new NixLazyScoped(new java.util.ArrayDeque<>(java.util.List.of((NixAttrset) globals.force())), new java.util.ArrayDeque<>()) {

			@Override
			public NixValue force() {
				/* head */

				NixAttrset rec = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

				return new NixLazyScoped(addToScope(scopes, rec), withs) {

					@Override
					public NixValue force() {


						rec.value.computeIfAbsent("c", k -> () -> NixInteger.create(1).createCall().force());
						return rec;
					}
				}.force();
			}
		}).createCall().force());

		System.out.println(new MainClosure(new java.util.ArrayDeque<>(java.util.List.of((NixAttrset) globals.force())), new java.util.ArrayDeque<>()).force());
	}
}