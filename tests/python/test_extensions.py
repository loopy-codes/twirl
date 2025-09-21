def test_extensions():
    import loom

    exts = loom.supported_extensions()

    # must be a list
    assert isinstance(exts, list)

    # must not be empty, with hardcoded length
    # to catch new feature additions in the future
    assert len(exts) == 11
