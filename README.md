# Rust-XMP-Metadata-Tool

A Rust-based CLI tool for reading and writing XMP photo metadata, powered by xmpkit.

## Usage

```command
# to read metadata
cargo run -- read <path> <field>
cargo run -- read path/to/img.jpg persons
cargo run -- read path/to/img.jpg regions

# to write metadata
cargo run -- write <path> <field> <value>
cargo run -- write path/to/img.jpg persons "My Name"
cargo run -- write path/to/img.jpg regions "Your Name"  # unimplimented

# to delete metadata
cargo run -- delete <path> <field> <value>
cargo run -- delete path/to/img.jpg persons "My Name"
cargo run -- delete path/to/img.jpg regions "Your Name"  # unimplimented

```

## Supported XMP metadata fields

### Person Shown in the Image

Namespace URI: http://iptc.org/std/Iptc4xmpExt/2008-02-29/

Prefix: `Iptc4xmpExt`

| Label | Property Name | Type | Description |
| --- | --- | --- | -- |
| Person Shown | [PersonInImage](https://www.iptc.org/std/photometadata/specification/IPTC-PhotoMetadata-2025.1.html#person-shown-in-the-image) | Bag (Unordered array of text) | Name of a person shown in the image.

#### Example XMP packet (XML)

```xml
<?xpacket begin="" id=""?>
 <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
   <rdf:Description rdf:about="" xmlns:Iptc4xmpExt="http://iptc.org/std/Iptc4xmpExt/2008-02-29/">
    <Iptc4xmpExt:PersonInImage>
     <rdf:Bag>
      <rdf:li>First_Name Last_Name</rdf:li>
     </rdf:Bag>
    </Iptc4xmpExt:PersonInImage>
   </rdf:Description>
  </rdf:RDF>
 </x:xmpmeta>
<?xpacket end="w"?>
```

### Metadata Working Group Regions schema

[The Metadata Working Group's schema](https://exiv2.org/tags-xmp-mwg-rs.html) for image region metadata.

Namespace URI: http://www.metadataworkinggroup.com/schemas/regions/

Prefix: `mwg-rs`

| Label | Property Name | Type | Description |
| --- | --- | --- | -- |
| Regions | Regions | RegionInfo | Main structure containing region based info. |
| Applied To Dimensions | AppliedToDimensions | Dimensions | Width and height of the image at the time of storing region data.
| Region List | RegionList | Bag Region | List of Region structures.
| Area | Area | Area | Bounding box area that describes the region.
| Type | Type | Closed Choice of Text | Type purpose of the region (Face, Pet, Focus, BarCode).
| Name | Name | Text | Name of content, name of person shown in image region.



#### Example XMP packet (XML)

```xml
<?xpacket begin="" id=""?>
    <x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="Adobe XMP Core 7.0">
        <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
            <rdf:Description rdf:about="" 
                            xmlns:mwg-rs="http://www.metadataworkinggroup.com/schemas/regions/"
                            xmlns:stDim="http://ns.adobe.com/xap/1.0/sType/Dimensions#"
                            xmlns:stArea="http://ns.adobe.com/xmp/sType/Area#">
                <mwg-rs:Regions rdf:parseType="Resource">
                    <mwg-rs:AppliedToDimensions rdf:parseType="Resource">
                        <stDim:h>1600</stDim:h>
                        <stDim:unit>pixel</stDim:unit>
                        <stDim:w>2400</stDim:w>
                    </mwg-rs:AppliedToDimensions>
                    <mwg-rs:RegionList>
                        <rdf:Bag>
                            <rdf:li rdf:parseType="Resource">
                                <mwg-rs:Area rdf:parseType="Resource">
                                    <stArea:h>0.0732</stArea:h>
                                    <stArea:unit>normalized</stArea:unit>
                                    <stArea:w>0.1085</stArea:w>
                                    <stArea:x>0.3358</stArea:x>
                                    <stArea:y>0.2803</stArea:y>
                                </mwg-rs:Area>
                                <mwg-rs:Name>Felix</mwg-rs:Name>
                                <mwg-rs:Type>Face</mwg-rs:Type>
                            </rdf:li>
                        </rdf:Bag>
                    </mwg-rs:RegionList>
                </mwg-rs:Regions>
            </rdf:Description>
        </rdf:RDF>
    </x:xmpmeta>
<?xpacket end="w"?>
```
