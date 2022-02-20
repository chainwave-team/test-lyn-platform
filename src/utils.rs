elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::types::ManagedBuffer;

const IMAGE_EXT: &str = ".png";
const IPFS_SCHEME: &str = "ipfs://";
const METADATA_KEY_NAME: &str = "metadata:";
const METADATA_FILE_EXTENSION: &str = ".json";
const ATTR_SEPARATOR: &str = ";";
const URI_SLASH: &str = "/";
const TAGS_KEY_NAME: &str = "tags:";

pub fn str_to_buffer<M: ManagedTypeApi>(string: &str) -> ManagedBuffer<M> {
    ManagedBuffer::new_from_bytes(string.as_bytes())
}

pub fn u32_to_buffer<M: ManagedTypeApi>(num: &u32) -> ManagedBuffer<M> {
    use alloc::string::ToString;
    ManagedBuffer::new_from_bytes(num.to_string().as_bytes())
}

/// Return the NFT item name like "Lynxee #420"
pub fn build_name<M: ManagedTypeApi>(token_name: &ManagedBuffer<M>, id: &u32) -> ManagedBuffer<M> {
    let mut name = ManagedBuffer::new();
    name.append(token_name);
    name.append(&str_to_buffer(" #"));
    name.append(&u32_to_buffer(id));
    name
}

/// Build a vector with the image uri inside
pub fn build_uris<M: ManagedTypeApi>(
    image_cid: &ManagedBuffer<M>,
    index: &u32,
) -> ManagedVec<M, ManagedBuffer<M>> {
    let mut uris = ManagedVec::new();
    let mut img_ipfs_uri = str_to_buffer(IPFS_SCHEME);
    img_ipfs_uri.append(&image_cid);
    img_ipfs_uri.append(&str_to_buffer(URI_SLASH));
    img_ipfs_uri.append(&u32_to_buffer(index));
    img_ipfs_uri.append(&str_to_buffer(IMAGE_EXT));

    uris.push(img_ipfs_uri);
    uris
}

/// Build the attributes Buffer including the metadata json
/// Format: tags:tag1,tag2;metadata:ipfsCID/1.json
pub fn build_attributes<M: ManagedTypeApi>(
    json_cid: &ManagedBuffer<M>,
    tags: &ManagedBuffer<M>,
    index: &u32,
) -> ManagedBuffer<M> {
    let mut attributes = ManagedBuffer::new();

    // metadata:cid;
    attributes.append(&str_to_buffer(METADATA_KEY_NAME));
    attributes.append(&json_cid);
    attributes.append(&str_to_buffer(URI_SLASH));
    attributes.append(&u32_to_buffer(index));
    attributes.append(&str_to_buffer(METADATA_FILE_EXTENSION));
    attributes.append(&str_to_buffer(ATTR_SEPARATOR));
    // tags:tag1,tag2
    attributes.append(&str_to_buffer(TAGS_KEY_NAME));
    attributes.append(&tags);

    attributes
}
