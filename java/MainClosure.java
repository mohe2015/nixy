import java.util.IdentityHashMap;

public class MainClosure extends NixLazyScoped {

	public MainClosure(java.util.ArrayDeque<NixAttrset> scopes, java.util.ArrayDeque<NixAttrset> withs) {
		super(scopes, withs);
	}

	public NixValue force() {
		/*
		rec {
		  a.b = a.c;
		  a = { c = 1; };
	  	}.a.b
		 */

		NixAttrset rec = (NixAttrset) NixAttrset.create(new java.util.IdentityHashMap<>()).force();

		return new NixLazyScoped(addToScope(scopes, rec), withs) {

			@Override
			public NixValue force() {
				((NixAttrset)rec.value.computeIfAbsent("a", k -> NixAttrset.create(new IdentityHashMap<>())).force()).value.put("b", () -> ((NixAttrset)findVariable(scopes, withs, "a").force()).value.get("c").force());

				((NixAttrset)rec.value.computeIfAbsent("a", k -> NixAttrset.create(new IdentityHashMap<>())).force()).value.put("c", NixInteger.create(1).createCall());

				/* body */
				return rec;
			}
		}.force();
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure(new java.util.ArrayDeque<>(java.util.List.of((NixAttrset) globals.force())), new java.util.ArrayDeque<>()).force());
	}
}