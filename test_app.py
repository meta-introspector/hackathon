#!/usr/bin/env python3
"""
Test script for app.py - validates without launching server
"""
import sys

def test_imports():
    """Test that all imports work"""
    try:
        import gradio as gr
        print("✓ gradio imported")
        return True
    except ImportError as e:
        print(f"✗ Import failed: {e}")
        return False

def test_syntax():
    """Test Python syntax"""
    try:
        with open('app.py', 'r') as f:
            compile(f.read(), 'app.py', 'exec')
        print("✓ Syntax valid")
        return True
    except SyntaxError as e:
        print(f"✗ Syntax error: {e}")
        return False

def test_function_exists():
    """Test that main function exists"""
    try:
        import app
        assert hasattr(app, 'create_game_interface')
        print("✓ create_game_interface() exists")
        return True
    except Exception as e:
        print(f"✗ Function test failed: {e}")
        return False

def test_interface_creation():
    """Test interface can be created without launching"""
    try:
        import app
        demo = app.create_game_interface()
        print("✓ Interface created successfully")
        print(f"  Components: {len(demo.blocks)}")
        return True
    except Exception as e:
        print(f"✗ Interface creation failed: {e}")
        return False

if __name__ == "__main__":
    print("🧪 Testing app.py\n")
    
    tests = [
        test_imports,
        test_syntax,
        test_function_exists,
        test_interface_creation,
    ]
    
    results = [test() for test in tests]
    
    print(f"\n{'='*50}")
    print(f"Results: {sum(results)}/{len(results)} tests passed")
    
    if all(results):
        print("✅ All tests passed!")
        sys.exit(0)
    else:
        print("❌ Some tests failed")
        sys.exit(1)
