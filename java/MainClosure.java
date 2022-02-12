public class MainClosure implements NixObject {

	public NixObject call(NixObject arg) {
		NixObject.ensureLazy(arg);
		((NixObject) (a) -> {
			NixObject.ensureLambda(a);
			return a.add(NixInteger.create(1));
		}).force();

		return ((NixObject) a -> {
			NixObject.ensureLambda(a);
			return a.add(NixInteger.create(1));
		}).force();
	}

	public static void main(String[] args) {
		System.out.println(new MainClosure().force());
	}
}
