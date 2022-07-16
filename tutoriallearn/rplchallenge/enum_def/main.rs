
// 機械学習でのモデルデータ作成用ツール
enum MlOnnxModelFrameWork {
    CaffeSecond,
    MxNet,
    PyTorch,
    Chainer
}


fn main() {

    let ml_framework = MlOnnxModelFrameWork::MxNet;
    let ml_description = value_in_description(ml_framework);
    println!("{}", ml_description);

}

fn value_in_description(ommxframework: MlOnnxModelFrameWork) -> String {
    let caffe_description = "カリフォルニア大学で、開発されていた機械学習サービス";
    let mxnet_description = "深層学習ライブラリ";
    let pytorch_description = "Pythonで使われる機械学習ライブラリ";
    let chainer_description = "Pythonで、微分や線形代数など機械学習用の数学をサポートしてくれるライブラリ";

    // フレームワークの各種説明
    match ommxframework {
        MlOnnxModelFrameWork::CaffeSecond => (*caffe_description).to_string(),
        MlOnnxModelFrameWork::MxNet => (*mxnet_description).to_string(),
        MlOnnxModelFrameWork::PyTorch => (*pytorch_description).to_string(),
        MlOnnxModelFrameWork::Chainer => (*chainer_description).to_string(),
    }
}