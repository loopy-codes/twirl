def test_extensions():
    import pearl

    exts = pearl.supported_extensions()
    assert isinstance(exts, list)
    assert exts
