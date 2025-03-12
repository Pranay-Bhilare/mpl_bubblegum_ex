defmodule MplBubblegumEx.TreeUnitTest do
  use ExUnit.Case
  alias MplBubblegumEx.Tree
  alias MplBubblegumEx.Native
  import TestHelpers

  describe "validation functions" do
    # test "validate_pubkey accepts valid pubkey" do
    #   # Valid base58 encoded pubkey
    #   valid_pubkey = test_pubkey()
    #   assert Native.validate_pubkey(valid_pubkey) == true
    # end

    # test "validate_pubkey rejects invalid pubkey" do
    #   invalid_pubkey = "not-a-valid-pubkey"
    #   assert Native.validate_pubkey(invalid_pubkey) == false
    # end

    test "validate_keypair accepts valid keypair" do
      valid_keypair = load_test_keypair()
      assert byte_size(valid_keypair) == 64
      assert Native.validate_keypair(valid_keypair) == true
    end

    test "validate_keypair rejects invalid keypair" do
      invalid_keypair = :crypto.strong_rand_bytes(32) # Wrong size
      assert Native.validate_keypair(invalid_keypair) == false
    end
  end

  describe "tree input validation" do
    test "create_tree_config rejects invalid keypair" do
      invalid_keypair = :crypto.strong_rand_bytes(32)
      result = Tree.create_tree_config(
        14, 64, invalid_keypair, load_test_,
        "https://api.devnet.solana.com"
      )
      assert {:error, "Keypair must be 64 bytes"} = result
    end

    test "create_tree_config rejects invalid pubkey" do
      keypair = load_test_keypair()
      merkle_keypair = load_test_merkle_keypair()
      result = Tree.create_tree_config(
        14, 64, keypair, "invalid-pubkey",merkle_keypair,
        "https://api.devnet.solana.com"
      )
      assert {:error, "Invalid pubkey: invalid-pubkey"} = result
    end

    test "create_tree_config rejects invalid depth/buffer combination" do
      keypair = load_test_keypair()
      result = Tree.create_tree_config(
        5, 10, keypair, test_pubkey(),
        "https://api.devnet.solana.com"
      )
      assert {:error, message} = result
      assert String.contains?(message, "Invalid depth/buffer combination")
    end
  end
end
