using Godot;
using System;

public partial class Button : Godot.Button
{
	GDScript MyGDScript = GD.Load<GDScript>("res://Scripts/Button.gd");

	public override void _Ready()
	{
		GodotObject myGDScript = (GodotObject)MyGDScript.New();
	}
}
