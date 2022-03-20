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



						((NixAttrset)rec.value.computeIfAbsent("a", k -> NixAttrset.create(new java.util.IdentityHashMap<>())).force()).value.computeIfAbsent("b", k -> NixAttrset.create(new java.util.IdentityHashMap<>())).force()).intern(), findVariable(scopes, withs, "a")findVariable(scopes, withs, "c") SELECT .createCall());((NixAttrset)rec.value.computeIfAbsent("a", k -> NixAttrset.create(new java.util.IdentityHashMap<>())).force())".intern(), (new NixLazy() {

							@Override
							public NixValue force() {
								/* head */

								NixAttrset rec = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

								return new NixLazyScoped(addToScope(scopes, rec), withs) {

									@Override
									public NixValue force() {

										((NixAttrset)rec.value.computeIfAbsent("c", k -> NixAttrset.create(new java.util.IdentityHashMap<>())).force())".intern(), NixInteger.create(1).createCall());}}).createCall());}}).createCall().force();
									}

									public static void main(String[] args) {
										System.out.println(new MainClosure(new java.util.ArrayDeque<>(java.util.List.of((NixAttrset) globals.force())), new java.util.ArrayDeque<>()).force());
									}
								}
