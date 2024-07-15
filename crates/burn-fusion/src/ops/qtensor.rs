use burn_tensor::{
    backend::Backend,
    ops::{QTensorOps, QuantizedTensor},
    Device, QuantizationStrategy, Shape, TensorData,
};

use crate::{client::FusionClient, Fusion, FusionBackend};

impl<B: FusionBackend> QTensorOps<Self> for Fusion<B> {
    fn q_from_data<const D: usize>(
        _data: TensorData,
        _device: &Device<Self>,
    ) -> QuantizedTensor<Self, D> {
        unimplemented!()
    }

    fn quantize<const D: usize>(
        _tensor: <Self as Backend>::FloatTensorPrimitive<D>,
        _strategy: &QuantizationStrategy,
    ) -> <Self as Backend>::QuantizedTensorPrimitive<D> {
        unimplemented!()
    }

    fn dequantize<const D: usize>(
        _tensor: <Self as Backend>::QuantizedTensorPrimitive<D>,
        _strategy: &QuantizationStrategy,
    ) -> <Self as Backend>::FloatTensorPrimitive<D> {
        unimplemented!()
    }

    fn q_shape<const D: usize>(tensor: &QuantizedTensor<Self, D>) -> Shape<D> {
        tensor.shape()
    }

    fn q_device<const D: usize>(tensor: &QuantizedTensor<Self, D>) -> Device<Self> {
        tensor.client.device().clone()
    }

    fn q_reshape<const D1: usize, const D2: usize>(
        _tensor: QuantizedTensor<Self, D1>,
        _shape: Shape<D2>,
    ) -> QuantizedTensor<Self, D2> {
        unimplemented!()
    }

    async fn q_into_data<const D: usize>(
        _tensor: QuantizedTensor<Self, D>,
        _strategy: QuantizationStrategy,
    ) -> TensorData {
        unimplemented!()
    }
}
