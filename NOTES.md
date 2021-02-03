## Tasks


### Task: image to pdf with thumbnails. __DONE__

1. create directories
2. open image
3. write to single page pdf with requested transforms (default: grayscale, alpha=25)
4. create content file
   * generate uuids for each page (only one for this task).
5. create metadata file
   * later version can allow putting directly into a folder
6. create pagedata file
   * "Blank" for each page
7. make thumbnail for each page with no alpha
   * question about the name for the thumbs. Should it be the uuid of the PDF page, or should it be numbered.

#### Expected options (bold for v1)

* file: filename of the input images  DONE
* dest: directory in which to write the data. If missing, a temp directory will be used, and deleted when the process exits.  __TODO__
* alpha(-a): alpha to apply to image (default: 25)  DONE
* rmpath: mount point of the Remarkable root directory. If present, copy the book into place when done. (default: None)  DONE
* install(-i): __TODO__ copy the files into place
* restart(-r): restart the xochitl app.  DONE
* parent(-p): __TODO__ put in the requested parent folder (by name, not ID)

### Task: Copy files/dirs to Remarkable


### Task: Grab all files for a book (by name?).




### Directory structure

extension      |	type      |	description
---------------|------------|-------------
               | directory	| annotations will be stored here
.cache	       | directory	| not sure but empty initially
.content       | file	      | information including uuids for each page
.highlights    | directory  | not sure but empty initially
.metadata      | file	      | name, dates, type
.pagedata      | file       | list of templates, one per page
.pdf           | file       | the actual PDF uploaded
.textconversion| directory  | for converted annotations; empty initially
.thumbnails    | directory  | 362x512 pixel jpeg, one per page in document numbered 0, 1, …, n-1

(information from https://www.ucl.ac.uk/~ucecesf/remarkable/#org9e00c33)

### Content file

```
{
    "coverPageNumber": 0,
    "dummyDocument": false,
    "extraMetadata": {},
    "fileType": "pdf",
    "fontName": "",
    "lineHeight": -1,
    "margins": 100,
    "orientation": "portrait",
    "pageCount": 1,
    "pages": [
        "dfda01c2-43b2-4ca4-be20-80a6d6c7b183"
    ],
    "textAlignment": "left",
    "textScale": 1,
    "transform": {
        "m11": 1,
        "m12": 0,
        "m13": 0,
        "m21": 0,
        "m22": 1,
        "m23": 0,
        "m31": 0,
        "m32": 0,
        "m33": 1
    }
}
```

__Fields to change:__
* pageCount
* pages

### Metadata file

```
{
    "deleted": false,
    "lastModified": "1612291212372",
    "lastOpenedPage": 0,
    "metadatamodified": false,
    "modified": false,
    "parent": "ba618e64-129f-4221-b888-c4d1cc2ba361",
    "pinned": false,
    "synced": true,
    "type": "DocumentType",
    "version": 2,
    "visibleName": "141952609_3919385148111861_8515095882140395662_n"
}
```

__Fields to change:__
* lastModified
* visibleName
