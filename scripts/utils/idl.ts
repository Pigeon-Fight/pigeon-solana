export const IDL = {
  address: "96khaF5rSYhuyWKFFGsMBjZtg49pSJAoSyxKN5FAjWEp",
  metadata: {
    name: "pigeon_battle",
    version: "0.1.0",
    spec: "0.1.0",
    description: "Created with Anchor",
  },
  instructions: [
    {
      name: "initialize",
      discriminator: [175, 175, 109, 31, 13, 152, 155, 237],
      accounts: [
        {
          name: "user",
          writable: true,
          signer: true,
        },
        {
          name: "mint",
          writable: true,
          signer: true,
        },
        {
          name: "mint_authority",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 117, 116, 104, 111, 114, 105, 116, 121],
              },
            ],
          },
        },
        {
          name: "metadata",
          writable: true,
        },
        {
          name: "master_edition",
          writable: true,
        },
        {
          name: "destination",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "account",
                path: "user",
              },
              {
                kind: "const",
                value: [
                  6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206,
                  235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140,
                  245, 133, 126, 255, 0, 169,
                ],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
            program: {
              kind: "const",
              value: [
                140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142,
                13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216,
                219, 233, 248, 89,
              ],
            },
          },
        },
        {
          name: "pigeon_config",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  112, 105, 103, 101, 111, 110, 95, 99, 111, 110, 102, 105, 103,
                ],
              },
            ],
          },
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "associated_token_program",
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        },
        {
          name: "token_metadata_program",
          address: "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
        },
      ],
      args: [],
    },
    {
      name: "mint_nft",
      discriminator: [211, 57, 6, 167, 15, 219, 35, 251],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "admin",
          writable: true,
        },
        {
          name: "pigeon_config",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  112, 105, 103, 101, 111, 110, 95, 99, 111, 110, 102, 105, 103,
                ],
              },
            ],
          },
        },
        {
          name: "nft_info_account",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [110, 102, 116, 95, 100, 97, 116, 97],
              },
              {
                kind: "arg",
                path: "nft_class",
              },
            ],
          },
        },
        {
          name: "mint",
          writable: true,
          signer: true,
        },
        {
          name: "destination",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "const",
                value: [
                  6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206,
                  235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140,
                  245, 133, 126, 255, 0, 169,
                ],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
            program: {
              kind: "const",
              value: [
                140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142,
                13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216,
                219, 233, 248, 89,
              ],
            },
          },
        },
        {
          name: "metadata",
          writable: true,
        },
        {
          name: "master_edition",
          writable: true,
        },
        {
          name: "mint_authority",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 117, 116, 104, 111, 114, 105, 116, 121],
              },
            ],
          },
        },
        {
          name: "collection_mint",
          writable: true,
        },
        {
          name: "nft_attributes_account",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 116, 116, 114, 105, 98, 117, 116, 101, 115],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "associated_token_program",
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        },
        {
          name: "token_metadata_program",
          address: "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
        },
      ],
      args: [
        {
          name: "nft_class",
          type: "u8",
        },
      ],
    },
    {
      name: "purchase_item",
      discriminator: [38, 91, 106, 119, 28, 153, 35, 183],
      accounts: [
        {
          name: "user",
          writable: true,
          signer: true,
        },
        {
          name: "admin",
          writable: true,
        },
        {
          name: "mint",
        },
        {
          name: "token",
          pda: {
            seeds: [
              {
                kind: "account",
                path: "user",
              },
              {
                kind: "const",
                value: [
                  6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206,
                  235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140,
                  245, 133, 126, 255, 0, 169,
                ],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
            program: {
              kind: "const",
              value: [
                140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142,
                13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216,
                219, 233, 248, 89,
              ],
            },
          },
        },
        {
          name: "nft_attributes_account",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 116, 116, 114, 105, 98, 117, 116, 101, 115],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "pigeon_config",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  112, 105, 103, 101, 111, 110, 95, 99, 111, 110, 102, 105, 103,
                ],
              },
            ],
          },
        },
        {
          name: "item_info",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [105, 116, 101, 109, 95, 100, 97, 116, 97],
              },
              {
                kind: "arg",
                path: "item_class",
              },
            ],
          },
        },
      ],
      args: [
        {
          name: "item_class",
          type: "u8",
        },
      ],
    },
    {
      name: "pve",
      discriminator: [246, 84, 155, 27, 172, 119, 181, 243],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "mint",
        },
        {
          name: "token",
          pda: {
            seeds: [
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "const",
                value: [
                  6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206,
                  235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140,
                  245, 133, 126, 255, 0, 169,
                ],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
            program: {
              kind: "const",
              value: [
                140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142,
                13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216,
                219, 233, 248, 89,
              ],
            },
          },
        },
        {
          name: "attributes",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 116, 116, 114, 105, 98, 117, 116, 101, 115],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
      ],
      args: [
        {
          name: "enemy",
          type: {
            defined: {
              name: "NftAttributes",
            },
          },
        },
      ],
    },
    {
      name: "pvp",
      discriminator: [191, 228, 136, 17, 36, 74, 184, 45],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "mint",
        },
        {
          name: "token",
          pda: {
            seeds: [
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "const",
                value: [
                  6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206,
                  235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140,
                  245, 133, 126, 255, 0, 169,
                ],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
            program: {
              kind: "const",
              value: [
                140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142,
                13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216,
                219, 233, 248, 89,
              ],
            },
          },
        },
        {
          name: "op_mint",
        },
        {
          name: "attributes",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 116, 116, 114, 105, 98, 117, 116, 101, 115],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "op_attributes",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 116, 116, 114, 105, 98, 117, 116, 101, 115],
              },
              {
                kind: "account",
                path: "op_mint",
              },
            ],
          },
        },
      ],
      args: [],
    },
    {
      name: "set_item_data",
      discriminator: [232, 18, 201, 64, 169, 36, 85, 44],
      accounts: [
        {
          name: "admin",
          writable: true,
          signer: true,
        },
        {
          name: "item_class_info",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [105, 116, 101, 109, 95, 100, 97, 116, 97],
              },
              {
                kind: "arg",
                path: "item_class",
              },
            ],
          },
        },
        {
          name: "pigeon_config",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  112, 105, 103, 101, 111, 110, 95, 99, 111, 110, 102, 105, 103,
                ],
              },
            ],
          },
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [
        {
          name: "item_class",
          type: "u8",
        },
        {
          name: "item_data",
          type: {
            defined: {
              name: "ItemClassInfo",
            },
          },
        },
      ],
    },
    {
      name: "set_nft_data",
      discriminator: [250, 72, 34, 115, 122, 6, 221, 22],
      accounts: [
        {
          name: "admin",
          writable: true,
          signer: true,
        },
        {
          name: "nft_class_info",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [110, 102, 116, 95, 100, 97, 116, 97],
              },
              {
                kind: "arg",
                path: "nft_class",
              },
            ],
          },
        },
        {
          name: "pigeon_config",
          pda: {
            seeds: [
              {
                kind: "const",
                value: [
                  112, 105, 103, 101, 111, 110, 95, 99, 111, 110, 102, 105, 103,
                ],
              },
            ],
          },
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [
        {
          name: "nft_class",
          type: "u8",
        },
        {
          name: "nft_data",
          type: {
            defined: {
              name: "NftClassInfo",
            },
          },
        },
      ],
    },
    {
      name: "upgrade_nft",
      discriminator: [224, 33, 200, 0, 99, 177, 135, 3],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "nft_attribute_account",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [97, 116, 116, 114, 105, 98, 117, 116, 101, 115],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "mint",
        },
        {
          name: "token",
          pda: {
            seeds: [
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "const",
                value: [
                  6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206,
                  235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140,
                  245, 133, 126, 255, 0, 169,
                ],
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
            program: {
              kind: "const",
              value: [
                140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142,
                13, 131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216,
                219, 233, 248, 89,
              ],
            },
          },
        },
      ],
      args: [
        {
          name: "points",
          type: {
            defined: {
              name: "AllocatedAttribute",
            },
          },
        },
      ],
    },
  ],
  accounts: [
    {
      name: "ItemClassInfo",
      discriminator: [154, 10, 177, 96, 13, 85, 205, 184],
    },
    {
      name: "NftAttributes",
      discriminator: [141, 98, 28, 46, 120, 31, 36, 185],
    },
    {
      name: "NftClassInfo",
      discriminator: [62, 47, 131, 102, 70, 151, 111, 38],
    },
    {
      name: "PigeonConfig",
      discriminator: [244, 133, 88, 9, 239, 73, 111, 201],
    },
  ],
  events: [
    {
      name: "MintEvent",
      discriminator: [197, 144, 146, 149, 66, 164, 95, 16],
    },
  ],
  errors: [
    {
      code: 6000,
      name: "InsufficientFunds",
      msg: "Insufficient funds",
    },
    {
      code: 6001,
      name: "Unauthorized",
      msg: "Unauthorized",
    },
    {
      code: 6002,
      name: "NotOwner",
      msg: "Not owner",
    },
    {
      code: 6003,
      name: "InvalidDefender",
      msg: "Incorrect Opponent",
    },
    {
      code: 6004,
      name: "InvalidPoints",
      msg: "Invalid points",
    },
  ],
  types: [
    {
      name: "AllocatedAttribute",
      type: {
        kind: "struct",
        fields: [
          {
            name: "attack",
            type: "u16",
          },
          {
            name: "defense",
            type: "u16",
          },
          {
            name: "speed",
            type: "u16",
          },
          {
            name: "max_hp",
            type: "u16",
          },
          {
            name: "max_energy",
            type: "u16",
          },
        ],
      },
    },
    {
      name: "ItemClassInfo",
      type: {
        kind: "struct",
        fields: [
          {
            name: "price",
            type: "u64",
          },
          {
            name: "heal_hp",
            type: "u16",
          },
          {
            name: "heal_energy",
            type: "u16",
          },
          {
            name: "boost_attack",
            type: "u16",
          },
          {
            name: "boost_defense",
            type: "u16",
          },
          {
            name: "boost_speed",
            type: "u16",
          },
        ],
      },
    },
    {
      name: "MintEvent",
      type: {
        kind: "struct",
        fields: [
          {
            name: "mint",
            type: "pubkey",
          },
        ],
      },
    },
    {
      name: "NftAttributes",
      type: {
        kind: "struct",
        fields: [
          {
            name: "class",
            type: "u8",
          },
          {
            name: "hp",
            type: "u16",
          },
          {
            name: "energy",
            type: "u16",
          },
          {
            name: "exp",
            type: "u16",
          },
          {
            name: "allocated_point",
            type: "u16",
          },
          {
            name: "attack",
            type: "u16",
          },
          {
            name: "defense",
            type: "u16",
          },
          {
            name: "speed",
            type: "u16",
          },
          {
            name: "max_hp",
            type: "u16",
          },
          {
            name: "max_energy",
            type: "u16",
          },
        ],
      },
    },
    {
      name: "NftClassInfo",
      type: {
        kind: "struct",
        fields: [
          {
            name: "price",
            type: "u64",
          },
          {
            name: "boost_attack",
            type: "u16",
          },
          {
            name: "boost_defense",
            type: "u16",
          },
          {
            name: "boost_speed",
            type: "u16",
          },
        ],
      },
    },
    {
      name: "PigeonConfig",
      type: {
        kind: "struct",
        fields: [
          {
            name: "admin_key",
            type: "pubkey",
          },
        ],
      },
    },
  ],
};
