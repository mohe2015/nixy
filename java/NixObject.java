interface NixObject {

	public NixObject call(NixObject arg);
	public String getType();
	public NixObject getArguments();
	public NixObject getAttr(String name);

}